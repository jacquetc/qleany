// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "automapper.h"

// QHash<QMetaType, std::function<QVariant(const QVariant&)>>
// AutoMapper::AutoMapper::conversions;
QHash<QMetaType, std::function<QVariant()>> FrontEnds::Tools::AutoMapper::getDefaultSiblingFunctions;
QHash<QPair<QMetaType, QMetaType>, std::function<QVariant()>> FrontEnds::Tools::AutoMapper::getSiblingFunctions;

QHash<QPair<QMetaType, QMetaType>, std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer, const QVariant &sourceValue)>>
    FrontEnds::Tools::AutoMapper::writerHash;

QHash<QPair<QMetaType, QMetaType>, std::function<bool(const QMetaProperty &destinationProperty, void *gadgetPointer, const QList<QVariant> &sourceList)>>
    FrontEnds::Tools::AutoMapper::writerForListHash;
QMutex FrontEnds::Tools::AutoMapper::mutex;