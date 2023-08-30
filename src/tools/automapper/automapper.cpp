// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#include "qleany/tools/automapper/automapper.h"

// QHash<QMetaType, std::function<QVariant(const QVariant&)>>
// AutoMapper::AutoMapper::conversions;
QHash<QMetaType, std::function<QVariant()>> Qleany::Tools::AutoMapper::AutoMapper::getSiblingFunctions;

QHash<QMetaType,
      std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer, const QVariant &sourceValue)>>
    Qleany::Tools::AutoMapper::AutoMapper::writerHash;

QHash<QMetaType, std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer,
                                    const QList<QVariant> &sourceList)>>
    Qleany::Tools::AutoMapper::AutoMapper::writerForListHash;
QMutex Qleany::Tools::AutoMapper::AutoMapper::mutex;

QVariant Qleany::Tools::AutoMapper::AutoMapper::implMap(const QVariant &source)
{
    auto getSiblingFunction = getSiblingFunctions.find(source.metaType());

    // list all metatypes in getSiblingFunctions
    for (auto it = getSiblingFunctions.begin(); it != getSiblingFunctions.end(); ++it)
    {
        qDebug() << it.key().name();
    }

    if (getSiblingFunction == getSiblingFunctions.end())
    {
        qWarning() << "No mapping found for type" << source.typeName();
    }
    qDebug() << "Entries of getSiblingFunctions while in implMap: "
             << AutoMapper::AutoMapper::getSiblingFunctions.count();

    QVariant destination = getSiblingFunction.value()();

    const QMetaObject *sourceMetaObject = source.metaType().metaObject();
    const QMetaObject *destinationMetaObject = destination.metaType().metaObject();

    // convert to actual type
    const void *sourcePointer = source.data();
    void *destinationPointer = destination.data();

    int propertyCount = sourceMetaObject->propertyCount();

    for (int i = 0; i < propertyCount; ++i)
    {
        QMetaProperty sourceProperty = sourceMetaObject->property(i);

        if (sourceProperty.isReadable())
        {
            int destinationPropertyIndex = destinationMetaObject->indexOfProperty(sourceProperty.name());

            if (destinationPropertyIndex >= 0)
            {
                const QVariant &value = sourceProperty.readOnGadget(sourcePointer);
                const QMetaProperty &destinationProperty = destinationMetaObject->property(destinationPropertyIndex);

                QVariant destinationValue;

                if (destinationProperty.isWritable() &&
                    QMetaType::canConvert(value.metaType(), destinationProperty.metaType()))
                {
                    bool success = destinationProperty.writeOnGadget(destinationPointer, value);

                    if (!success)
                    {
                        qWarning() << "Failed to write value" << value << "to destination property"
                                   << destinationProperty.name();
                    }
                }

                else if (destinationProperty.isWritable())
                {
                    // Check if a conversion function exists for this
                    // property type
                    auto getSiblingFunction = getSiblingFunctions.find(value.metaType());

                    if (getSiblingFunction != getSiblingFunctions.end())
                    {
                        // We have a conversion for this type.
                        if (QString(value.metaType().name()).startsWith("QList<"))
                        {
                            // If it's a QList<QVariant>, process each
                            // QVariant.
                            QList<QVariant> sourceList = value.toList();

                            if (sourceList.isEmpty())
                            {
                                // If the list is empty, we can't get the
                                // type of the custom type.
                                // So we can't instantiate a new object of
                                // the custom type.
                                // So we can't call mapImpl recursively.
                                // So we can't convert the list.
                                // So we can't do anything.
                                // So we just return an empty list.
                                continue;
                            }

                            // destinationValue = foreignDestinationList;

                            auto writeForListIt = writerForListHash.find(value.metaType());

                            if (writeForListIt != writerForListHash.end())
                            {
                                bool success =
                                    writeForListIt.value()(destinationProperty, destinationPointer, sourceList);

                                if (!success)
                                {
                                    qWarning() << "Failed to write value" << destinationValue
                                               << "to destination property" << destinationProperty.name();
                                }
                            }
                        }
                        else
                        {
                            // It's a single QVariant with custom type.

                            auto writeIt = writerHash.find(value.metaType());

                            if (writeIt != writerHash.end())
                            {
                                bool success = writeIt.value()(destinationProperty, destinationPointer, value);

                                if (!success)
                                {
                                    qWarning() << "Failed to write value" << destinationValue
                                               << "to destination property" << destinationProperty.name();
                                }
                            }
                        }
                    }
                }

                else
                {
                    qCritical("AutoMapper error: Property types do not match. Check the types of the source and "
                              "destination properties.");
                }
            }
        }
    }
    return destination;
}
