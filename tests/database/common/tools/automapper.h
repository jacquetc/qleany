// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QDateTime>
#include <QHash>
#include <QMutexLocker>
#include <QVariant>
#include <QtCore/QMetaObject>
#include <QtCore/QMetaProperty>

namespace DatabaseTest::Tools
{
using namespace Qt::Literals::StringLiterals;

class AutoMapper
{
  public:
    using MetaTypePair = QPair<QMetaType, QMetaType>;

    template <class SourceType, class DestinationType>
    static void registerMapping(bool isDefault = false, bool reverseConversion = false)
    {
        QMutexLocker locker(&mutex);

        qRegisterMetaType<SourceType>();
        qRegisterMetaType<DestinationType>();

        // simple value return
        getSiblingFunctions[MetaTypePair(QMetaType::fromType<SourceType>(), QMetaType::fromType<DestinationType>())] =
            []() { return QVariant::fromValue(DestinationType()); };

        if (isDefault)
        {
            getDefaultSiblingFunctions[QMetaType::fromType<SourceType>()] = []() {
                return QVariant::fromValue(DestinationType());
            };
        }

        // simple return for list
        getSiblingFunctions[MetaTypePair(QMetaType::fromType<QList<SourceType>>(),
                                         QMetaType::fromType<QList<DestinationType>>())] = []() {
            return QVariant::fromValue(DestinationType());
        };
        if (isDefault)
        {
            getDefaultSiblingFunctions[QMetaType::fromType<QList<SourceType>>()] = []() {
                return QVariant::fromValue(DestinationType());
            };
        }

        writerHash[MetaTypePair(QMetaType::fromType<SourceType>(), QMetaType::fromType<DestinationType>())] =
            [&](const QMetaProperty &destinationProperty, void *gadgetPointer, const QVariant &sourceValue) {
                const QVariant &customDestinationObject = AutoMapper::implDefaultMap<DestinationType>(sourceValue);
                DestinationType destinationValue = customDestinationObject.value<DestinationType>();

                return destinationProperty.writeOnGadget(gadgetPointer, QVariant::fromValue(destinationValue));
            };

        writerForListHash[MetaTypePair(QMetaType::fromType<QList<SourceType>>(),
                                       QMetaType::fromType<DestinationType>())] =
            [](const QMetaProperty &destinationProperty, void *gadgetPointer, const QList<QVariant> &sourceList) {
                QList<DestinationType> destinationList;

                for (const QVariant &value : sourceList)
                {
                    const QVariant &customDestinationObject = AutoMapper::implDefaultMap<DestinationType>(value);
                    destinationList.append(customDestinationObject.value<DestinationType>());
                }

                auto variant = QVariant::fromValue(destinationList);
                return destinationProperty.writeOnGadget(gadgetPointer, variant);
            };

        if (reverseConversion)
        {
            getSiblingFunctions[MetaTypePair(QMetaType::fromType<DestinationType>(),
                                             QMetaType::fromType<SourceType>())] = []() {
                return QVariant::fromValue(SourceType());
            };

            if (isDefault)
            {
                getDefaultSiblingFunctions[QMetaType::fromType<DestinationType>()] = []() {
                    return QVariant::fromValue(SourceType());
                };
            }

            getSiblingFunctions[MetaTypePair(QMetaType::fromType<QList<DestinationType>>(),
                                             QMetaType::fromType<SourceType>())] = []() {
                return QVariant::fromValue(SourceType());
            };
            if (isDefault)
            {
                getDefaultSiblingFunctions[QMetaType::fromType<QList<DestinationType>>()] = []() {
                    return QVariant::fromValue(SourceType());
                };
            }

            writerHash[MetaTypePair(QMetaType::fromType<DestinationType>(), QMetaType::fromType<SourceType>())] =
                [](const QMetaProperty &destinationProperty, void *gadgetPointer, const QVariant &sourceValue) {
                    const QVariant &customDestinationObject = AutoMapper::implDefaultMap<SourceType>(sourceValue);
                    SourceType destinationValue = customDestinationObject.value<SourceType>();

                    return destinationProperty.writeOnGadget(gadgetPointer, QVariant::fromValue(destinationValue));
                };

            writerForListHash[MetaTypePair(QMetaType::fromType<QList<DestinationType>>(),
                                           QMetaType::fromType<SourceType>())] =
                [](const QMetaProperty &destinationProperty, void *gadgetPointer, const QList<QVariant> &sourceList) {
                    QList<SourceType> destinationList;

                    for (const QVariant &value : sourceList)
                    {
                        const QVariant &customDestinationObject = AutoMapper::implDefaultMap<SourceType>(value);
                        destinationList.append(customDestinationObject.value<SourceType>());
                    }
                    auto variant = QVariant::fromValue(destinationList);
                    return destinationProperty.writeOnGadget(gadgetPointer, variant);
                };
        }

        // qDebug() << "Entries after registering: " << AutoMapper::AutoMapper::getSiblingFunctions.count();
    }

    template <class SourceType, class DestinationType> static DestinationType map(const SourceType &sourceObject)
    {
        const QVariant &destinationVariant = implMap<SourceType, DestinationType>(sourceObject);

        return destinationVariant.value<DestinationType>();
    }

    template <class SourceType, class DestinationType> static DestinationType map(SourceType &sourceObject)
    {
        const QVariant &destinationVariant = implMap<SourceType, DestinationType>(sourceObject);

        return destinationVariant.value<DestinationType>();
    }

  private:
    template <class DestinationType> static QVariant implDefaultMap(const QVariant &source)
    {

        // list all metatypes in getSiblingFunctions
        //        for (auto it = getSiblingFunctions.begin(); it != getSiblingFunctions.end(); ++it)
        //        {
        //            qDebug() << "from" << it.key().first.name() << "to" << it.key().second.name();
        //        }

        //        qDebug() << "Entries of getSiblingFunctions while in implMap: "
        //                 << AutoMapper::AutoMapper::getSiblingFunctions.count();

        QVariant destinationVariant = QVariant::fromValue(DestinationType());

        const QMetaObject *sourceMetaObject = source.metaType().metaObject();
        const QMetaObject *destinationMetaObject = destinationVariant.metaType().metaObject();

        // convert to actual type
        const void *sourcePointer = source.data();
        void *destinationPointer = destinationVariant.data();

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
                    const QMetaProperty &destinationProperty =
                        destinationMetaObject->property(destinationPropertyIndex);

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

                    else
                    {
                        qCritical("AutoMapper error: Property types do not match. Check the types of the source and "
                                  "destination properties.");
                    }
                }
            }
        }
        return destinationVariant;
    }

    template <class SourceType, class DestinationType> static QVariant implMap(const SourceType &source)
    {

        // list all metatypes in getSiblingFunctions
        //        for (auto it = getSiblingFunctions.begin(); it != getSiblingFunctions.end(); ++it)
        //        {
        //            qDebug() << "from" << it.key().first.name() << "to" << it.key().second.name();
        //        }
        //        for (auto it = getDefaultSiblingFunctions.begin(); it != getDefaultSiblingFunctions.end(); ++it)
        //        {
        //            qDebug() << "Default from" << it.key().name() << "to" << it.value()().typeName();
        //        }

        //        qDebug() << "Entries of getSiblingFunctions while in implMap: "
        //                 << AutoMapper::AutoMapper::getSiblingFunctions.count();

        QVariant destinationVariant = QVariant::fromValue(DestinationType());
        QVariant sourceVariant = QVariant::fromValue(source);

        const QMetaObject *sourceMetaObject = sourceVariant.metaType().metaObject();
        const QMetaObject *destinationMetaObject = destinationVariant.metaType().metaObject();

        // convert to actual type
        const void *sourcePointer = sourceVariant.data();
        void *destinationPointer = destinationVariant.data();

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
                    const QMetaProperty &destinationProperty =
                        destinationMetaObject->property(destinationPropertyIndex);

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
                        auto getDefaultSiblingFunction = getDefaultSiblingFunctions.find(value.metaType());

                        if (getDefaultSiblingFunction != getDefaultSiblingFunctions.end())
                        {
                            // We have a conversion for this type.
                            if (QString::fromLatin1(value.metaType().name()).startsWith("QList<"_L1))
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

                                auto writeForListIt = writerForListHash.find(
                                    MetaTypePair(value.metaType(), getDefaultSiblingFunction.value()().metaType()));

                                if (writeForListIt != writerForListHash.end() /*&&
                                    source.metaData().getSet(sourceProperty.name())*/)
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

                                auto writeIt = writerHash.find(
                                    MetaTypePair(value.metaType(), getDefaultSiblingFunction.value()().metaType()));

                                if (writeIt != writerHash.end() &&
                                    source.metaData().getSet(QString::fromLatin1(sourceProperty.name())))
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
        return destinationVariant;
    }

  private:
    static QHash<QMetaType, std::function<QVariant()>> getDefaultSiblingFunctions;
    static QHash<MetaTypePair, std::function<QVariant()>> getSiblingFunctions;
    static QHash<MetaTypePair, std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer,
                                                  const QVariant &sourceValue)>>
        writerHash;
    static QHash<MetaTypePair, std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer,
                                                  const QList<QVariant> &sourceList)>>
        writerForListHash;

    static QMutex mutex;
};
} // namespace DatabaseTest::Tools