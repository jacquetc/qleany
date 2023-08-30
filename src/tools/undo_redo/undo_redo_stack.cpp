// Copyright (c) 2023 Cyril Jacquet
// This file is part of Qleany which is released under MIT License.
// See file LICENSE for full license details.
#include "qleany/tools/undo_redo/undo_redo_stack.h"

using namespace Qleany::Tools::UndoRedo;

UndoRedoStack::UndoRedoStack(QObject *parent) : QObject{parent}, m_id{QUuid()}, m_currentIndex(-1)
{
}

UndoRedoStack::UndoRedoStack(QObject *parent, const QUuid &id) : QObject{parent}, m_id{id}, m_currentIndex(-1)
{
}

QQueue<QSharedPointer<UndoRedoCommand>> &UndoRedoStack::queue()
{
    // return reference to m_queue
    return m_queue;
}

void UndoRedoStack::setQueue(const QQueue<QSharedPointer<UndoRedoCommand>> &newQueue)
{
    m_queue = newQueue;
}

QUuid UndoRedoStack::id() const
{
    return m_id;
}

void UndoRedoStack::setId(const QUuid &newId)
{
    m_id = newId;
}

int UndoRedoStack::currentIndex() const
{
    return m_currentIndex;
}

void UndoRedoStack::setCurrentIndex(int newCurrentIndex)
{
    m_currentIndex = newCurrentIndex;
}

void UndoRedoStack::incrementCurrentIndex()
{
    m_currentIndex++;
}

void UndoRedoStack::decrementCurrentIndex()
{
    m_currentIndex--;
}
